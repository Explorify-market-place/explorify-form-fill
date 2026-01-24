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

async function readPdf(key) {
    let res = await fetch("https://jeubsu5h36ulifpl7c3gw7jci40msjya.lambda-url.ap-south-1.on.aws/", {
        method: "POST",
        headers: {
            "Content-Type": "application/json;charset=UTF-8"
        },
        body: JSON.stringify({
            pdf: key
        })
    })
    res = await res.text()
    return JSON.parse(res)
}
readLink("https://traveltechindia.netlify.app/Details/Pachmarhi").then(data => console.log(data))
// readPdf("itineraries/temp-ec705976-8753-40a3-ad11-80a92909bae1-1769014010233-spiti.pdf").then(data => console.log(data))
