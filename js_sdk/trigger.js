async function trigger(eventName) {
    let headers = new Headers();
    headers.append("Event-Name", "MyEvent")
    let result = await fetch("http://localhost:7950/?x=c", {
        method: "POST",
        headers: headers
    });
}
trigger("MyEvent")