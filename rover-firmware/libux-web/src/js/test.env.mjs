window.MirageJS = {};
window.MirageJS.Server = undefined;

export async function injectMirage() {
    let script = document.createElement("script");
    script.type = "text/javascript";
    let result = new Promise(resolve => {
        script.onload = function() { initMirage(); resolve(); }
        script.src = "https://unpkg.com/miragejs@0.1.48/dist/mirage-umd.js";
    });

    document.head.append(script);

    return result;
}

export function initMirage() {
    let { createServer } = window.MirageJS.Server;

    createServer({
        routes() {
            this.urlPrefix = "http://rover";
            this.namespace = "api";

            this.post("move", (schema, request) => { /* no response intended */ });
        }
    });
}