const { createApp, ref } = Vue

const userId = getCookie("id")

const login = () => {
    let id = getCookie("id")
    if (id === "");

    fetch("/api/user/login").then()
}

fetch(`/api/user/search?id=${userId}`, { method: "GET" }).then(res => res.json()).then(user => {
    console.log(user)
    createApp({
        setup() {
            const name = ref(user.data.users[0])
            return { name }
        }
    }).mount('#name')

    createApp({
        setup() {
            const id = ref(user.id)
            return { id }
        }
    }).mount('#id')
})
