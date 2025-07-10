class rDatabase {
    #url
    /**
     * 
     * @param {string} url 
     * @returns {rDatabase}
     */
    constructor(url) {
        this.#url =url;
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
        let response = await fetch(this.#url+"/"+keyName, {
            method: "GET"
        });
        return await response.text();
    }
    /**
     * 
     * @param {string} keyName
     */
    /**
     * 
     * Deletes a key from the database
     */
    async delete(keyName) {
        let response = await fetch(this.#url+"/"+keyName, {
            method: "DELETE"
        });
        if (!response.ok) {
            throw new Error("Cannot delete value that does not exist.");
        }
    }
    /**
     * @param {string} keyName
     */
    /**
     * 
     * Changes a value in the database, creating it if it does not exist.
     */
    async change(keyName, keyValue) {
        let response = await fetch(`${this.#url}/${keyName}=${keyValue}`, {
            method: "POST"
        });
        if (!response.ok) {
            throw new Error("Cannot change the specified value.");
        }
    }
}
let rdb = new rDatabase("http://localhost:7950");
let v = await rdb.change("a", "b");