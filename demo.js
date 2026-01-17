let res = await fetch("https://jeubsu5h36ulifpl7c3gw7jci40msjya.lambda-url.ap-south-1.on.aws/", {
    method: "POST",
    headers: {
        "Content-Type": "application/json;charset=UTF-8"
    },
    body: JSON.stringify({
        url: "https://vinaiak.com"
    })
})
res = await res.text()
console.log(JSON.parse(res))
