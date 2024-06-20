const progress = document.getElementById("progress");

import init, * as czv from "../../pkg/czv.js";
// Must run `await init()` or `initSync()` first for web use
await init();

const fileReader = new FileReader();

fileReader.onloadstart = () => {
    if (progress) progress.style.display = "block";
};

fileReader.onloadend = () => {
    const rowCountElement = document.getElementById("row-count");
    const columnCountElement = document.getElementById("column-count");
    if (rowCountElement)
        rowCountElement.innerText = String(
            czv.rowCount({
                file_data: fileReader.result as string,
            })
        );
    if (columnCountElement)
        columnCountElement.innerText = String(
            czv.columnCount({ file_data: fileReader.result as string })
        );
    if (progress) progress.style.display = "none";
};

const input = document.getElementById("upload");
if (input)
    input.addEventListener("change", () => {
        // @ts-ignore
        fileReader.readAsText(input.files[0]);
    });
