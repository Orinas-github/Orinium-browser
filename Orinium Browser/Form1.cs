﻿using System;
using System.Windows.Forms;

namespace Orinium_Browser
{
    public partial class Form1 : Form
    {
        private WebBrowser webBrowser; // WebBrowser コントロールを宣言
        private TextBox urlTextBox; // 入力用 TextBox を宣言
        private Button goButton; // 移動ボタンを宣言

        public Form1()
        {
            InitializeComponent(); // フォームの初期化
            InitializeBrowser(); // ブラウザの初期化
        }

        private void InitializeBrowser()
        {
            // WebBrowser コントロールの初期設定
            webBrowser = new WebBrowser
            {
                Dock = DockStyle.Fill // フォームにフィットするようにドッキング
            };
            this.Controls.Add(webBrowser); // フォームのコントロールに追加

            // URL 入力用 TextBox の設定
            urlTextBox = new TextBox
            {
                Dock = DockStyle.Top // 上部にドッキング
            };
            urlTextBox.KeyDown += urlTextBox_KeyDown; // エンターキーのイベントを追加
            this.Controls.Add(urlTextBox); // フォームのコントロールに追加

            // 移動ボタンの設定
            goButton = new Button
            {
                Text = "移動",
                Dock = DockStyle.Top // 上部にドッキング
            };
            goButton.Click += GoButton_Click; // ボタンクリックイベントを追加
            this.Controls.Add(goButton); // フォームのコントロールに追加
        }

        // エンターキーが押されたときの処理
        private void urlTextBox_KeyDown(object sender, KeyEventArgs e)
        {
            if (e.KeyCode == Keys.Enter) // エンターキーかチェック
            {
                GoToUrl(); // URL 移動を呼び出し
            }
        }

        private void GoButton_Click(object sender, EventArgs e)
        {
            GoToUrl(); //ページ遷移
        }


        private void GoToUrl()
        {
            // 入力された URL に移動
            if (!string.IsNullOrWhiteSpace(urlTextBox.Text))
            {
                webBrowser.Navigate(urlTextBox.Text); // WebBrowser に URL をナビゲート
            }
            else
            {
                MessageBox.Show("URL を入力してください！", "エラー", MessageBoxButtons.OK, MessageBoxIcon.Warning);
            }
        }
    }
}