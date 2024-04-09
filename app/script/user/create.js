const create = () => {
    let name = document.getElementById("name").value
    let id = document.getElementById("id").value
    let password = document.getElementById("password").value
    let introduction = document.getElementById("introduction").value

    if (name === "") { document.getElementById("message").textContent = "名前が空白です"; return }
    if (id === "") { document.getElementById("message").textContent = "IDが空白です"; return }
    if (password === "") { document.getElementById("message").textContent = "パスワードが空白です"; return }

    fetch("/api/user/create", postHeader({ name: name, id: id, password: password, introduction: introduction })).then(r => r.json()).then(res => {
        if (res.status === true) window.location.href = "/"
        else document.getElementById("message").textContent = res.data
    })
}
