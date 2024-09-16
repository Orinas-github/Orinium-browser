using System.Windows.Forms;

namespace Orinium_Browser
{
    internal class OriniumBrowser_main : Control
    {
        private WebBrowser webBrowser;

        public OriniumBrowser_main(bool isIE_mode)
        {
            SetMode(isIE_mode);
        }

        public void SetMode(bool isIE_mode)
        {
            // 画面をクリアするために初期化
            this.Controls.Clear();

            if (isIE_mode)
            {
                webBrowser = new WebBrowser
                {
                    Dock = DockStyle.Fill // ウィンドウにフィットさせる
                };
                this.Controls.Add(webBrowser); // MyWebBrowser に WebBrowser を追加
                if (webBrowser != null && webBrowser.Url != null)
                {
                    Navigate(webBrowser.Url.ToString(), isIE_mode);
                }
            }
            else
            {
                // IE_mode でない場合の別の処理
                Label messageLabel = new Label
                {
                    Text = "IE Mode is disabled!",
                    AutoSize = true,
                    Dock = DockStyle.Fill,
                    TextAlign = System.Drawing.ContentAlignment.MiddleCenter // 中央揃え
                };
                this.Controls.Add(messageLabel); // メッセージを追加

                // 他の独自機能をここに追加できるよ〜♪
            }
        }

        public void Navigate(string url, bool isIE_mode)
        {
            webBrowser.Navigate(url); // 指定した URL に移動
        }
    }
}