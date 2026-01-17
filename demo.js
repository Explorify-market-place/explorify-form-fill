const fs = require('fs').promises;

async function readLink(link) {
    let res = await fetch("https://jeubsu5h36ulifpl7c3gw7jci40msjya.lambda-url.ap-south-1.on.aws/", {
        method: "POST",
        headers: {
            "Content-Type": "application/json;charset=UTF-8"
        },
        body: JSON.stringify({
            link,
            water: "galat field maaf kar dega"
        })
    })
    res = await res.text()
    return JSON.parse(res)
}

async function pdfToBase64(filePath) {
    try {
        const fileBuffer = await fs.readFile(filePath);
        return fileBuffer.toString('base64');
    } catch (error) {
        console.error("Error reading PDF:", error);
        throw error;
    }
}
async function readPdf(path) {
    let res = await fetch("https://jeubsu5h36ulifpl7c3gw7jci40msjya.lambda-url.ap-south-1.on.aws/", {
        method: "POST",
        headers: {
            "Content-Type": "application/json;charset=UTF-8"
        },
        body: JSON.stringify({
            pdf: await pdfToBase64(path)
        })
    })
    res = await res.text()
    return JSON.parse(res)
}
// readLink("https://traveltechindia.netlify.app/Details/Pachmarhi").then(data => console.log(data))
readPdf("Winter Spiti Valley - Delhi to Delhi.pdf").then(data => console.log(data))
