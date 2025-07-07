async function trigger(eventName) {
    let headers = new Headers();
    headers.append("Event-Name", "MyEvent")
    let result = fetch("http://localhost:7950/trigger", {
        method: "POST",
        headers: headers
    });
}
trigger("MyEvent")