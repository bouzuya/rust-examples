# std1

std::io::stdout に対して BufWriter を使うべきかを確認するために書いた例。

結論は基本的に使うべき。特に `'\n'` が含まれると速度に影響が出る (おそらく `'\n'` ごとに flush されている) 。
