# Expected-Output

## Diesel

```
getPlayer: Some(Player { id: "test", coins: Some(1), goods: Some(1) })
countPlayers: 1920
print 1 player: Player { id: "test", coins: Some(1), goods: Some(1) }
print 2 player: Player { id: "16e8539a-58bf-4259-9197-57b2967ade75", coins: Some(10000), goods: Some(10000) }
print 3 player: Player { id: "ecb60313-e851-4a5d-ad39-bbf899c5f082", coins: Some(10000), goods: Some(10000) }

buyGoods:
    => this trade will fail

buyGoods:
    => this trade will success

[buyGoods]:
    'trade success'
```


## Http Request

```bash
loop to create 10 players:
1111111111

get player 1:
null

get players by limit 3:
[{"coins":100,"goods":20,"id":"2e86517c-5cc2-4148-ac61-45bac68ad558"},{"coins":100,"goods":20,"id":"8509a96c-fa93-43aa-a0cd-1648bb24fe80"},{"coins":100,"goods":20,"id":"a48dccad-12b8-49c3-b040-18ec3ba237cb"}]

get players count:
10

trade by two players:

```
