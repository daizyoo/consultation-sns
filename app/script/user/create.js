const create = () => {
    const name = document.getElementById('name').value;
    const id = document.getElementById('id').value;
    const password = document.getElementById('password').value;

    if (name === '') { document.getElementById('message').textContent = '名前を入力してください'; return; }
    if (id === '') { document.getElementById('message').textContent = 'idを入力してください'; return; }
    if (password === '') { document.getElementById('message').textContent = 'パスワードを入力してください'; return; }

    fetch("/api/user/create", postHeader()).then(res => res.json()).then(res => {
        if (res.status == true) window.location.href = '/';
        else document.getElementById('message').textContent = res.data;
    })
}