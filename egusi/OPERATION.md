websocket message handler: holds a usersession

socket message example  const raw = `{"op":"${opcode}","p":${JSON.stringify(data)}${ ref ? `,"ref":"${ref}"` : ""}}`;