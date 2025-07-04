// Delete key x
let response = await fetch("http://127.0.0.1:7950/x", { method: "DELETE" });
console.log(response.status)