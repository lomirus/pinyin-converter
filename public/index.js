import init, { convert_pinyin_ascii } from "../pkg/pinyin_converter.js";
// import init, { convert_pinyin_ascii } from "./pkg/pinyin_converter.js";

const inputBox = document.querySelector("#input");
const outputBox = document.querySelector("#output");

await init();

inputBox.addEventListener('input', (e) => {
    try {
        outputBox.value = convert_pinyin_ascii(e.target.value, 1);
        outputBox.classList.remove('error')
    } catch {
        outputBox.classList.add('error')
    }
})