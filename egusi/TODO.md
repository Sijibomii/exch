create a websocket contract basically messages the websocket server expects and the replies(if any) that the client should expect

At login or first connect rust app sends user details(like wallet balance etc) of logged in user to rabbitmq and elixir takes it and stores it in app session
One websocket connection, elixir app spins up a usersession for that user using details from appstate

Each ticker has a ticker session. that keeps track of the current state of the order book and can give to a user on request

websocket messages are ansered by the user session. If it's a message making a trade, the session for that ticker is called


possible rabbit msgs

new ticker
incemental
snapshot
new order
new login