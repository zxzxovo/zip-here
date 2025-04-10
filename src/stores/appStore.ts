import { defineStore } from "pinia"
import { Ref, ref } from "vue"
import { LangType } from "../i18n/type"

const useAppStore = defineStore("app", () => {
    const appLang: Ref<LangType> = ref("zh-CN")

    const setLang = (newLang: LangType) => {
        appLang.value = newLang;
    }

    return {
        appLang,
        setLang,
    }
})

export default useAppStore
