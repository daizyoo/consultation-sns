const login = () => {
    let id = document.getElementById("id").value
    let password = document.getElementById("password").value

    if (id === "") { document.getElementById("message").textContent = "IDが空白です"; return }
    if (password === "") { document.getElementById("message").textContent = "パスワードが空白です"; return }

    fetch("/api/user/login", postHeader({ id: id, password: password })).then(res => res.json()).then(res => {
        console.log(res)
    })
}