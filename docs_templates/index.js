AOS.init();
function download() {
    let platform = navigator.platform;
    if (platform === "Win32") {
        console.log("Downloading for windows")
        window.open("./weather.exe", "_blank");
    }
    else {
        console.log("Downloading for unix")
        window.open("./weather", "_blank");
    }
}

function download_unix() {
    window.open("./weather", "_blank");
}

function download_windows() {
    window.open("./weather.exe", "_blank");
}
