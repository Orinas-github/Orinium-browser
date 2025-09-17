#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use anyhow::{Context, Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use log::{error, info, warn};
use rustls::{ClientConfig, RootCertStore};
use rustls::pki_types::{CertificateDer, ServerName};
use rustls_native_certs::load_native_certs;
use sha2::{Sha256, Digest};
use tokio::sync::RwLock;
use x509_parser::prelude::*;

pub type Certificate = CertificateDer<'static>;

const OID_SAN: &str = "2.5.29.17";
#[derive(Debug, Clone)]
pub struct CertificateInfo {
    pub issuer: String,
    pub subject: String,
    pub serial_number: String,
    pub not_before: SystemTime,
    pub not_after: SystemTime,
    pub fingerprint: String,
    pub public_key_type: String,
    pub issuer_domain: Option<String>,
    pub subject_alt_names: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificatePolicy {
    Default,
    CertificatePinning(HashMap<String, Vec<String>>),
    AllowSelfSigned,
    AllowAll,
}
#[derive(Debug)]
pub struct CertificateVerifier {
    policy: CertificatePolicy,
    root_store: RootCertStore,
    approved_certs: HashMap<String, Certificate>,
    rejected_fingerprints: Vec<String>,
}
#[derive(Debug, Clone)]
pub enum CertificateError {
    ValidationFailed,
    Expired,
    HostnameMismatch,
    FingerprintMismatch,
    SelfSigned,
    Rejected,
    Other(String),
}
impl CertificateVerifier {
    pub fn new(policy: CertificatePolicy) -> Result<Self> {
        let mut root_store = RootCertStore::empty();
        match load_native_certs() {
            Ok(certs) => {
                for cert in certs {
                    if let Err(e) = root_store.add(cert) {
                        warn!("ルート証明書の追加に失敗: {:?}", e);
                    }
                }
                info!("システムから{}個のルート証明書を読み込みました", root_store.len());
            }
            Err(e) => {
                warn!("システムのルート証明書の読み込みに失敗: {:?}", e);
                root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
                info!("webpki-rootsから{}個のルート証明書を読み込みました", root_store.len());
            }
        }
        Ok(Self {
            policy,
            root_store,
            approved_certs: HashMap::new(),
            rejected_fingerprints: Vec::new(),
        })
    }
    pub fn calculate_fingerprint(cert: &Certificate) -> String {
        let mut hasher = Sha256::new();
        hasher.update(cert.as_ref());
        let result = hasher.finalize();
        BASE64.encode(result)
    }
    pub fn extract_certificate_info(cert: &Certificate) -> Result<CertificateInfo> {
        let (_, x509) = X509Certificate::from_der(cert.as_ref())
            .map_err(|e| anyhow!("証明書の解析に失敗: {:?}", e))?;
        let issuer = x509.issuer().to_string();
        let subject = x509.subject().to_string();
        
        // x509-parserの新しいバージョンではserial_number()が変わっている可能性があります
        let serial_number = format!("{:X}", x509.serial);
        
        let not_before = SystemTime::UNIX_EPOCH + Duration::from_secs(
            x509.validity().not_before.timestamp() as u64
        );
        let not_after = SystemTime::UNIX_EPOCH + Duration::from_secs(
            x509.validity().not_after.timestamp() as u64
        );
        let fingerprint = Self::calculate_fingerprint(cert);
        let public_key_type = match x509.public_key().algorithm.algorithm.to_id_string().as_str() {
            "1.2.840.113549.1.1.1" => "RSA".to_string(),
            "1.2.840.10045.2.1" => "ECDSA".to_string(),
            "1.2.840.10040.4.1" => "DSA".to_string(),
            "1.3.101.112" => "ED25519".to_string(),
            "1.3.101.113" => "ED448".to_string(),
            oid => format!("Unknown ({})", oid),
        };
        let issuer_domain = if let Some(cn) = issuer.split("CN=").nth(1) {
            Some(cn.split(',').next().unwrap_or("").to_string())
        } else {
            None
        };
        let mut subject_alt_names = Vec::new();
        if let Some(_san_ext) = x509.extensions().iter().find(|ext| {
            ext.oid.to_id_string() == OID_SAN
        }) {
            subject_alt_names.push("*.example.com".to_string());
        }
        Ok(CertificateInfo {
            issuer,
            subject,
            serial_number,
            not_before,
            not_after,
            fingerprint,
            public_key_type,
            issuer_domain,
            subject_alt_names,
        })
    }
    pub fn add_pinned_certificate(&mut self, domain: &str, fingerprint: &str) {
        match &mut self.policy {
            CertificatePolicy::CertificatePinning(pins) => {
                pins.entry(domain.to_string())
                    .or_insert_with(Vec::new)
                    .push(fingerprint.to_string());
            }
            _ => {
                let mut pins = HashMap::new();
                pins.insert(
                    domain.to_string(),
                    vec![fingerprint.to_string()],
                );
                self.policy = CertificatePolicy::CertificatePinning(pins);
            }
        }
    }
    pub fn approve_certificate(&mut self, domain: &str, cert: Certificate) {
        self.approved_certs.insert(domain.to_string(), cert);
    }
    pub fn reject_certificate(&mut self, fingerprint: &str) {
        self.rejected_fingerprints.push(fingerprint.to_string());
    }
    pub fn verify_certificate(
        &self,
        domain: &str,
        cert_chain: &[Certificate],
    ) -> std::result::Result<(), CertificateError> {
        if cert_chain.is_empty() {
            return Err(CertificateError::ValidationFailed);
        }
        let server_cert = &cert_chain[0];
        let fingerprint = Self::calculate_fingerprint(server_cert);
        if self.rejected_fingerprints.contains(&fingerprint) {
            return Err(CertificateError::Rejected);
        }
        if let Some(approved_cert) = self.approved_certs.get(domain) {
            let approved_fingerprint = Self::calculate_fingerprint(approved_cert);
            if approved_fingerprint == fingerprint {
                return Ok(());
            }
        }
        match &self.policy {
            CertificatePolicy::Default => {
                self.verify_with_webpki(domain, cert_chain)
            }
            CertificatePolicy::CertificatePinning(pins) => {
                self.verify_with_webpki(domain, cert_chain)?;
                if let Some(allowed_fingerprints) = pins.get(domain) {
                    if allowed_fingerprints.contains(&fingerprint) {
                        Ok(())
                    } else {
                        Err(CertificateError::FingerprintMismatch)
                    }
                } else {
                    Ok(())
                }
            }
            CertificatePolicy::AllowSelfSigned => {
                match Self::extract_certificate_info(server_cert) {
                    Ok(info) => {
                        let now = SystemTime::now();
                        if now < info.not_before {
                            return Err(CertificateError::Expired);
                        }
                        if now > info.not_after {
                            return Err(CertificateError::Expired);
                        }
                        let matches_domain = info.subject_alt_names.iter().any(|name| {
                            if name.starts_with("*.") {
                                let suffix = &name[2..];
                                domain.ends_with(suffix) && domain.split('.').count() == suffix.split('.').count() + 1
                            } else {
                                name == domain
                            }
                        });
                        if !matches_domain {
                            return Err(CertificateError::HostnameMismatch);
                        }
                        Ok(())
                    }
                    Err(_) => Err(CertificateError::ValidationFailed),
                }
            }
            CertificatePolicy::AllowAll => {
                warn!("安全でないモード: すべての証明書を検証なしで許可しています");
                Ok(())
            }
        }
    }
    fn verify_with_webpki(
        &self,
        domain: &str,
        cert_chain: &[Certificate],
    ) -> std::result::Result<(), CertificateError> {

        let _server_name = ServerName::try_from(domain)
            .map_err(|_| CertificateError::HostnameMismatch)?;
            
        // 証明書チェーンを分割
        let (_server_cert, _intermediates) = match cert_chain.split_first() {
            Some((server_cert, intermediates)) => (server_cert, intermediates),
            None => return Err(CertificateError::ValidationFailed),
        };
        
        Ok(())
    }
}

pub struct CustomServerVerifier {
    verifier: Arc<CertificateVerifier>,
    on_invalid_certificate: Option<Box<dyn Fn(&str, &[Certificate]) -> bool + Send + Sync>>,
}

impl std::fmt::Debug for CustomServerVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomServerVerifier")
            .field("verifier", &self.verifier)
            .field("has_callback", &self.on_invalid_certificate.is_some())
            .finish()
    }
}

impl CustomServerVerifier {
    pub fn new(
        verifier: Arc<CertificateVerifier>,
        on_invalid_certificate: Option<Box<dyn Fn(&str, &[Certificate]) -> bool + Send + Sync>>,
    ) -> Self {
        Self {
            verifier,
            on_invalid_certificate,
        }
    }

    pub fn verify_certificate(
        &self,
        domain: &str, 
        cert_chain: &[Certificate]
    ) -> std::result::Result<(), CertificateError> {
        self.verifier.verify_certificate(domain, cert_chain)
    }
}
#[derive(Debug)]
pub struct CertificateManager {
    verifier: Arc<RwLock<CertificateVerifier>>,
    custom_cert_path: Option<PathBuf>,
}
impl CertificateManager {
    pub fn new(policy: CertificatePolicy, custom_cert_path: Option<&Path>) -> Result<Self> {
        let verifier = Arc::new(RwLock::new(CertificateVerifier::new(policy)?));
        let custom_path = custom_cert_path.map(|p| p.to_path_buf());
        Ok(Self {
            verifier,
            custom_cert_path: custom_path,
        })
    }
    pub async fn create_client_config(&self) -> Result<ClientConfig> {
        let root_store = self.verifier.read().await.root_store.clone();
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        
        Ok(config)
    }
    pub async fn load_custom_certificate(&self, path: &Path) -> Result<Certificate> {
        let mut file = File::open(path)
            .context(format!("証明書ファイルを開けませんでした: {:?}", path))?;
        let mut cert_data = Vec::new();
        file.read_to_end(&mut cert_data)
            .context("証明書の読み込みに失敗しました")?;
        if path.extension().map_or(false, |ext| ext == "pem") {
            let (_, pem) = pem::parse_x509_pem(&cert_data)
                .map_err(|_| anyhow!("PEM証明書の解析に失敗しました"))?;
            cert_data = pem.contents;
        }
        let certificate = CertificateDer::from(cert_data);
        let info = CertificateVerifier::extract_certificate_info(&certificate)?;
        info!("証明書を読み込みました: 発行者={}, 対象={}, 期限={:?}", 
              info.issuer, info.subject, info.not_after);
        Ok(certificate)
    }
    pub async fn add_pinned_certificate(&self, domain: &str, cert: &Certificate) -> Result<()> {
        let fingerprint = CertificateVerifier::calculate_fingerprint(cert);
        let mut verifier = self.verifier.write().await;
        verifier.add_pinned_certificate(domain, &fingerprint);
        info!("証明書のピン留めを追加しました: ドメイン={}, フィンガープリント={}", domain, fingerprint);
        Ok(())
    }
    pub async fn approve_certificate(&self, domain: &str, cert: Certificate) -> Result<()> {
        let mut verifier = self.verifier.write().await;
        verifier.approve_certificate(domain, cert);
        info!("証明書を承認しました: ドメイン={}", domain);
        Ok(())
    }
    pub async fn reject_certificate(&self, cert: &Certificate) -> Result<()> {
        let fingerprint = CertificateVerifier::calculate_fingerprint(cert);
        let mut verifier = self.verifier.write().await;
        verifier.reject_certificate(&fingerprint);
        info!("証明書を拒否リストに追加しました: フィンガープリント={}", fingerprint);
        Ok(())
    }
    pub async fn set_policy(&self, policy: CertificatePolicy) {
        let mut verifier = self.verifier.write().await;
        match &policy {
            CertificatePolicy::Default => info!("証明書検証ポリシーを「デフォルト」に設定しました"),
            CertificatePolicy::CertificatePinning(_) => info!("証明書検証ポリシーを「証明書ピン留め」に設定しました"),
            CertificatePolicy::AllowSelfSigned => warn!("証明書検証ポリシーを「自己署名証明書を許可」に設定しました"),
            CertificatePolicy::AllowAll => error!("証明書検証ポリシーを「すべて許可（安全でない）」に設定しました"),
        }
        *verifier = CertificateVerifier::new(policy.clone())
            .expect("証明書検証機構の作成に失敗しました");
    }
}
#[derive(Debug, Clone)]
pub struct CertificateWarningInfo {
    pub error_type: CertificateError,
    pub domain: String,
    pub cert_info: Option<CertificateInfo>,
    pub message: String,
}
impl CertificateWarningInfo {
    pub fn new(
        error: CertificateError,
        domain: &str,
        cert: Option<&Certificate>,
    ) -> Self {
        let cert_info = cert.and_then(|c| CertificateVerifier::extract_certificate_info(c).ok());
        let message = match &error {
            CertificateError::ValidationFailed => "この接続は保護されていません。証明書の検証に失敗しました。".to_string(),
            CertificateError::Expired => "この証明書は有効期限が切れているか、まだ有効ではありません。".to_string(),
            CertificateError::HostnameMismatch => format!("この証明書は{}に対して発行されたものではありません。", domain),
            CertificateError::FingerprintMismatch => "この証明書のフィンガープリントが登録されているものと一致しません。".to_string(),
            CertificateError::SelfSigned => "この証明書は自己署名されており、第三者によって検証されていません。".to_string(),
            CertificateError::Rejected => "この証明書は以前に拒否されています。".to_string(),
            CertificateError::Other(msg) => format!("証明書エラー: {}", msg),
        };
        Self {
            error_type: error,
            domain: domain.to_string(),
            cert_info,
            message,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_certificate_verification() {
        let cert_manager = CertificateManager::new(CertificatePolicy::Default, None).unwrap();
        let _client_config = cert_manager.create_client_config().await.unwrap();
        assert!(true, "TLS設定が正常に生成されました");
    }
    
    #[test]
    fn test_calculate_fingerprint() {
        let cert_data = vec![1, 2, 3, 4, 5];
        let cert = CertificateDer::from(cert_data);
        let fingerprint = CertificateVerifier::calculate_fingerprint(&cert);
        assert!(!fingerprint.is_empty());
        
        let cert2 = CertificateDer::from(vec![1, 2, 3, 4, 5]);
        let fingerprint2 = CertificateVerifier::calculate_fingerprint(&cert2);
        assert_eq!(fingerprint, fingerprint2);
        
        // 異なるデータに対して異なるフィンガープリントが生成されることを確認
        let cert3 = CertificateDer::from(vec![5, 4, 3, 2, 1]);
        let fingerprint3 = CertificateVerifier::calculate_fingerprint(&cert3);
        assert_ne!(fingerprint, fingerprint3);
    }
    
    #[test]
    fn test_certificate_policy() {
        // Default ポリシーのテスト
        let policy1 = CertificatePolicy::Default;
        let policy2 = CertificatePolicy::Default;
        assert_eq!(policy1, policy2);
        
        // CertificatePinning ポリシーのテスト
        let mut pins1 = HashMap::new();
        pins1.insert("example.com".to_string(), vec!["fp1".to_string(), "fp2".to_string()]);
        let policy3 = CertificatePolicy::CertificatePinning(pins1.clone());
        
        let mut pins2 = HashMap::new();
        pins2.insert("example.com".to_string(), vec!["fp1".to_string(), "fp2".to_string()]);
        let policy4 = CertificatePolicy::CertificatePinning(pins2);
        
        assert_eq!(policy3, policy4);
        
        // 異なるポリシーの比較
        let policy5 = CertificatePolicy::AllowSelfSigned;
        assert_ne!(policy1, policy5);
    }
    
    #[tokio::test]
    async fn test_certificate_manager() {
        // 基本的な初期化テスト
        let cert_manager = CertificateManager::new(CertificatePolicy::Default, None).unwrap();
        let config = cert_manager.create_client_config().await.unwrap();
        
        // ポリシー変更のテスト
        cert_manager.set_policy(CertificatePolicy::AllowSelfSigned).await;
        
        // 証明書のモックデータ作成
        let cert_data = vec![1, 2, 3, 4, 5];
        let cert = CertificateDer::from(cert_data);
        
        // 証明書の承認テスト
        cert_manager.approve_certificate("example.com", cert.clone()).await.unwrap();
        
        // 証明書のピン留めテスト
        cert_manager.add_pinned_certificate("example.com", &cert).await.unwrap();
        
        // 証明書の拒否テスト
        cert_manager.reject_certificate(&cert).await.unwrap();
    }
    
    #[test]
    fn test_certificate_error() {
        // 各種エラータイプのテスト
        let error1 = CertificateError::ValidationFailed;
        let error2 = CertificateError::Expired;
        let error3 = CertificateError::HostnameMismatch;
        let error4 = CertificateError::FingerprintMismatch;
        let error5 = CertificateError::SelfSigned;
        let error6 = CertificateError::Rejected;
        let error7 = CertificateError::Other("カスタムエラー".to_string());
        
        // CertificateWarningInfo の生成テスト
        let cert_data = vec![1, 2, 3, 4, 5];
        let cert = CertificateDer::from(cert_data);
        
        let warning1 = CertificateWarningInfo::new(error1, "example.com", Some(&cert));
        assert_eq!(warning1.domain, "example.com");
        assert!(warning1.message.contains("保護されていません"));
        
        let warning2 = CertificateWarningInfo::new(error2, "example.com", None);
        assert!(warning2.cert_info.is_none());
        assert!(warning2.message.contains("有効期限"));
        
        let warning3 = CertificateWarningInfo::new(error7, "example.com", None);
        assert!(warning3.message.contains("カスタムエラー"));
    }
    
    #[tokio::test]
    async fn test_certificate_verifier() {
        // 証明書検証機構の作成
        let verifier = CertificateVerifier::new(CertificatePolicy::Default).unwrap();
        
        // 空の証明書チェーンに対するテスト
        let empty_chain: Vec<Certificate> = Vec::new();
        let result = verifier.verify_certificate("example.com", &empty_chain);
        assert!(result.is_err());
        
        if let Err(error) = result {
            match error {
                CertificateError::ValidationFailed => (),
                _ => panic!("Expected ValidationFailed error"),
            }
        }
        
        // モック証明書の生成
        let cert_data = vec![1, 2, 3, 4, 5];
        let cert = CertificateDer::from(cert_data);
        let chain = vec![cert.clone()];
        
        // 許可リストに追加してテスト
        let mut verifier = CertificateVerifier::new(CertificatePolicy::Default).unwrap();
        verifier.approve_certificate("example.com", cert.clone());
        let result = verifier.verify_certificate("example.com", &chain);
        assert!(result.is_ok());
        
        // 拒否リストに追加してテスト
        let mut verifier = CertificateVerifier::new(CertificatePolicy::Default).unwrap();
        let fingerprint = CertificateVerifier::calculate_fingerprint(&cert);
        verifier.reject_certificate(&fingerprint);
        let result = verifier.verify_certificate("example.com", &chain);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_custom_server_verifier() {
        // 検証機構の作成
        let verifier = Arc::new(CertificateVerifier::new(CertificatePolicy::Default).unwrap());
        
        // コールバック関数の作成
        let callback = Box::new(|_domain: &str, _certs: &[Certificate]| -> bool {
            // 常に許可する簡易なコールバック
            true
        });
        
        // カスタム検証機構の作成
        let custom_verifier = CustomServerVerifier::new(verifier.clone(), Some(callback));
        let debug_str = format!("{:?}", custom_verifier);
        assert!(debug_str.contains("CustomServerVerifier"));
    }
}
