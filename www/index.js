import * as wasm from "interpreter";
import hello_world_basic from "!!binary-loader!../example_app/hello_world_basic.elf"
//import { entry } from "./webpack.config";

let array = new Uint8Array(hello_world_basic.length);
for (let i = 0; i < hello_world_basic.length; i++) {
    array[i] = hello_world_basic.charCodeAt(i);
}

let emulator = wasm.Emulator.new()

let entry_point = emulator.load_and_link(array);
if (entry_point > 0) {
    console.log(`Elf linked successfully. Entrypoint is ${entry_point}.`);
    emulator.set_esp(0x40000000);

    for (let i = 0 ; i <1000 ; i++) {
        emulator.step();
    }
} else {
    console.log("Elf failed linking.");
}