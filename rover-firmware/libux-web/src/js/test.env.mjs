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
    let { createServer, Response } = window.MirageJS.Server;

    createServer({
        routes() {
            this.urlPrefix = "http://rover";
            this.namespace = "api";

            let distance = 0;

            this.post("move", (schema, request) => {
                return new Response(204);
            });
            this.post("look", (schema, request) => {
                return new Response(204);
            });
            this.get("sense/obstacles", (schema, request) => {
                return [Math.random() > 0.5, Math.random() < 0.5];
            }, { timing: Math.random() * 3000 });
            this.get("sense/lines", (schema, request) => {
                return [Math.random() > 0.5, Math.random() < 0.5];
            }, { timing: Math.random() * 3000 });
            this.get("sense/distance", (schema, request) => {
                distance += (Math.random() > 0.5 ? 1 : -1) * Math.floor(Math.random() * 100);

                return distance;
            }, { timing: Math.random() * 3000 });
        }
    });
}