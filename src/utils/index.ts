import { i18nTexts } from "../i18n";
import langKeys from "../i18n/langKeys";
import { LangType } from "../i18n/type";
import useAppStore from "../stores/appStore";

export default function useI18n(key: langKeys): string {

    const appStore = useAppStore();
    const langUsing = appStore.appLang as LangType;
    return i18nTexts[langUsing][key] || i18nTexts["en"][key];
}