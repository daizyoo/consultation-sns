const { createApp, ref } = Vue

fetch("/api/user/login").then(res => res.json()).then(res => {
    console.log(res)

    if (res.status === false) {
        window.location.href = "/user/create.html"
        return
    }

    const user = res.data;

    createApp({
        setup() {
            const id = ref(user.id)
            return { id }
        }
    }).mount('#id')

    createApp({
        setup() {
            const name = ref(user.name)
            return { name }
        }
    }).mount('#name')

    createApp({
        setup() {
            const introduction = ref(user.introduction)
            return { introduction }
        }
    }).mount('#introduction')
});
