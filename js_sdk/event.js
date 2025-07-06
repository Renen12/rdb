/**
 * 
 * @param {string} eventName 
 */
async function subscribe(eventName) {
    let headers = new Headers();
    headers.append("Event-Name", "MyEvent")
    let result = fetch("http://localhost:7950/subscribe", {
        method: "POST",
        headers: headers
    });
}
subscribe("test_event");