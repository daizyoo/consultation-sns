const postHeader = (data) => {
    return {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    }
}

const getCookie = (name) => {
    return document.cookie
        .split("; ")
        .find((row) => row.startsWith(name))
        .split("=")[1]
}
