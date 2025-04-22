import {ref} from "vue";
import {NavigationGuard} from "vue-router";
export const leaveTimer = ref<number>(0)
export const isLeaving = ref(false)
export const navigationGuard: NavigationGuard = (to, from, next) => {
    isLeaving.value = true
    console.log(to,from)
    leaveTimer.value = window.setTimeout(() => {
        next();
        isLeaving.value = false
    }, 200)

    const handler = () => {
        window.clearTimeout(leaveTimer.value)
        next()
    }

    document.addEventListener('animationend', handler, { once: true })
}

