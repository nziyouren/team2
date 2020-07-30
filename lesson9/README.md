## 第九课作业

**(7 分)**

利用 off-chain worker，试从两个或以上的加密货币价格提供者 (最少两个)，透过其 API 取得 ETH 币的价格，
取其平均值，然后把它推 (append) 到 ocw 链下储存的 Vec 内。

加密货币价格提供者包括以下：
  - https://coinmarketcap.com/
  - https://coincap.io/
  - https://www.cryptocompare.com/
  - 其他你自己找到的提供者也可以

**(3 分)** 

附加题：为你的 ocw 写单元测试

coincap:
{
    "data":{
        "id":"ethereum",
        "rank":"2",
        "symbol":"ETH",
        "name":"Ethereum",
        "supply":"111837254.2490000000000000",
        "maxSupply":null,
        "marketCapUsd":"26636475399.5136100069260518",
        "volumeUsd24Hr":"1078370273.6335106592245216",
        "priceUsd":"238.1717575094327905",
        "changePercent24Hr":"1.7488000276419087",
        "vwap24Hr":"237.8206683612338757"
    },
    "timestamp":1595260763853
}
