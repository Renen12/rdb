export class rDatabase {
    #url
    /**
     * 
     * @param {string} url 
     * @returns {rDatabase}
     */
    constructor(url) {
        this.#url = url;
        return this;
    }
    /**
     * 
     * @param {string} keyName 
     * @returns {string}
     */
    /**
     * 
     * Returns a key from the database.
     */
    async get(keyName) {
        let response = await fetch(this.#url + "/" + keyName, {
            method: "GET"
        });
        return await response.text();
    }
    /**
     * 
     * Deletes a key from the database
     */
    async delete(keyName) {
        let response = await fetch(this.#url + "/" + keyName, {
            method: "DELETE"
        });
        if (!response.ok) {
            throw new Error("Cannot delete a value that does not exist.");
        }
    }
    /**
     * @param {string} keyName
     */
    /**
     * 
     * Changes a value in the database, creating it if it does not exist.
     *  Supply eventName if you want your change to also trigger an event for subscribers to that event.
     */
    async change(keyName, keyValue, eventName) {
        let headers = [];
        if (eventName !== null) {
            headers = [["Event-Name", eventName]];
        }
        let response = await fetch(`${this.#url}/change?${keyName}=${keyValue}`, {
            method: "POST",
            headers: headers
        });
        if (!response.ok) {
            throw new Error("Cannot change the specified value.");
        }
    }
    /**
     * 
     * @param {string} eventName 
     * @param {(response: Response) => {}} toRunWhenTriggered 
     */
     subscribe(eventName, toRunWhenTriggered) {
        let headers = new Headers();
        headers.append("Event-Name", eventName)
        let result = fetch("http://localhost:7950/subscribe", {
            method: "POST",
            headers: headers
        });
        result.then((v) => {
            toRunWhenTriggered(v);
        });
    }
}
export default rDatabase;