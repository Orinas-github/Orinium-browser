using Orinium_Browser;
using System;
using System.Windows.Forms;

namespace Orinium_Browser
{
    static class Program
    {
        // アプリケーションのメインエントリーポイント
        [STAThread] // シングルスレッドアパートメントスタイルを指定
        static void Main()
        {
            Application.EnableVisualStyles(); // ビジュアルスタイルを有効化
            Application.SetCompatibleTextRenderingDefault(false); // テキストレンダリングを設定

            // Form1 を新しく作成して、アプリケーションを実行
            Application.Run(new Form1());
        }
    }
}