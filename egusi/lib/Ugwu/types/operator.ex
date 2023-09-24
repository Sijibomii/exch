import EctoEnum

alias Ugwu.Message.Auth
alias Ugwu.Message.Trade
alias Ugwu.Message.Request

defenum(
  Ugwu.Message.Types.Operator,
  [
    # auth: 0..10
    {Auth.Request, 1},

    # trade 11..20
    {Trade.New, 11},
    {Trade.Modify, 12},
    {Trade.Cancel, 13},
    {Trade.Listen, 14},

    # request 21..30
    {Request.Orderbook, 21},
    {Request.Join, 22},
    {Request.Snapshot, 23}
  ]
)
