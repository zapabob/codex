# Page snapshot

```yaml
- generic [ref=e3]:
  - generic [ref=e6]:
    - heading "このサイトにアクセスできません" [level=1] [ref=e7]
    - paragraph [ref=e8]:
      - strong [ref=e9]: localhost
      - text: で接続が拒否されました。
    - generic [ref=e10]:
      - paragraph [ref=e11]: 次をお試しください
      - list [ref=e12]:
        - listitem [ref=e13]: 接続を確認する
        - listitem [ref=e14]:
          - link "プロキシとファイアウォールを確認する" [ref=e15] [cursor=pointer]:
            - /url: "#buttons"
    - generic [ref=e16]: ERR_CONNECTION_REFUSED
  - generic [ref=e17]:
    - button "再読み込み" [ref=e19] [cursor=pointer]
    - button "詳細" [ref=e20] [cursor=pointer]
```