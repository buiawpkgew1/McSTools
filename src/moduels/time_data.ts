import { ref, watchEffect, onUnmounted } from 'vue'
export class TimeManager {
    private _currentTime = ref('')
    private _currentDate = ref('')
    private _hours = ref('')
    private _minutes = ref('')
    private _seconds = ref('')
    private _isNewSecond = ref(false)
    private timerId: number | null = null

    constructor(autoStart = true) {
        if (autoStart) {
            this.start()
        }
    }

    public get currentTime() {
        return this._currentTime.value
    }

    public get currentDate() {
        return this._currentDate.value
    }

    public get hours() {
        return this._hours.value
    }

    public get minutes() {
        return this._minutes.value
    }

    public get seconds() {
        return this._seconds.value
    }

    public get isNewSecond() {
        return this._isNewSecond.value
    }

    private update() {
        const now = new Date()
        const newHours = now.getHours().toString().padStart(2, '0')
        const newMinutes = now.getMinutes().toString().padStart(2, '0')
        const newSeconds = now.getSeconds().toString().padStart(2, '0')

        if (this._seconds.value !== newSeconds) {
            this._isNewSecond.value = true
            setTimeout(() => (this._isNewSecond.value = false), 300)
        }

        if (this._hours.value !== newHours) this._hours.value = newHours
        if (this._minutes.value !== newMinutes) this._minutes.value = newMinutes
        this._seconds.value = newSeconds

        this._currentTime.value = now.toLocaleTimeString('zh-CN', {
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit'
        })

        this._currentDate.value = now.toLocaleDateString('zh-CN', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        })
    }

    public start() {
        if (!this.timerId) {
            this.update()
            this.timerId = window.setInterval(() => this.update(), 1000)
        }
    }

    public stop() {
        if (this.timerId) {
            clearInterval(this.timerId)
            this.timerId = null
        }
    }

    public useInComponent() {
        const setup = () => {
            onUnmounted(() => this.stop())
            watchEffect(() => this.start())
        }

        return {
            currentTime: this._currentTime,
            currentDate: this._currentDate,
            hours: this._hours,
            minutes: this._minutes,
            seconds: this._seconds,
            isNewSecond: this._isNewSecond,
            setup
        }
    }
}

export const globalTimeManager = new TimeManager()

export const createTimeManager = (autoStart = true) => {
    return new TimeManager(autoStart)
}