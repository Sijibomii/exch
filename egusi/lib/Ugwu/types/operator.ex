import EctoEnum

alias Ugwu.Message.Auth
alias Ugwu.Message.Trade
alias Uqwu.Message.Request

defenum(
  Ugwu.Message.Types.Operator,
  [
    # auth: 0..10
    {Auth.Request, 1},

    # trade 11..20
    {Trade.New, 11},
    {Trade.Modify, 12},
    {Trade.Cancel, 13},

    # request 21..30
    {Request.Join, 21},
    {Request.Snapshot, 22}
  ]
)
