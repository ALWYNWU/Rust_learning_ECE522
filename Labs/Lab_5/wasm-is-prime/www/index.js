import * as wasm from "wasm-is-prime";
const textbox1= document.getElementById("PrimeNumber");
document.getElementById("CheckNumber").addEventListener("click", event => {
    wasm.CheckPrime(textbox1.value);
});

