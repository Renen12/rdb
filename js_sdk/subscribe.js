/**
 * 
 * @param {string} eventName 
 */
async function subscribe(eventName) {
    let headers = new Headers();
    headers.append("Event-Name", "MyEvent")
    let result = await fetch("http://localhost:7950/subscribe", {
        method: "POST",
        headers: headers
    });
    console.log(await result.text());
}
subscribe("test_event");