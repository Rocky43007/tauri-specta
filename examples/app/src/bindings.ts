         // This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

         export const commands = {
/**
 * HELLO
 * WORLD
 * !!!!
 */
async helloWorld(myName: string) : Promise<string> {
return await TAURI_INVOKE("plugin:tauri-specta|hello_world", { myName });
},
async goodbyeWorld() : Promise<string> {
return await TAURI_INVOKE("plugin:tauri-specta|goodbye_world");
},
async hasError() : Promise<__Result__<string, number>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:tauri-specta|has_error") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async someStruct() : Promise<MyStruct> {
return await TAURI_INVOKE("plugin:tauri-specta|some_struct");
},
async generic() : Promise<null> {
return await TAURI_INVOKE("plugin:tauri-specta|generic");
}
}

export const events = __makeEvents__<{
demoEvent: DemoEvent,
emptyEvent: EmptyEvent
}>({
demoEvent: "plugin:tauri-specta:demo-event",
emptyEvent: "plugin:tauri-specta:empty-event"
})

/** user-defined types **/

export type DemoEvent = string
export type EmptyEvent = null
export type MyStruct = { some_field: string }

/** tauri-specta globals **/

         import { invoke as TAURI_INVOKE } from "@tauri-apps/api";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindowHandle as __WebviewWindowHandle__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: T extends null
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

type __Result__<T, E> =
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>
) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindowHandle__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindowHandle__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case "listen":
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case "once":
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case "emit":
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    }
  );
}

     