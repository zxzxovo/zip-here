import { ref, computed } from 'vue';
import en from "./en";
import { I18nKeyType, I18nType, LangType } from "./type";
import zhCN from "./zh-CN";

// 语言文本对象
export const i18nTexts = {
  "en": en,
  "zh-CN": zhCN
} as I18nType;

// 默认语言
const defaultLang = 'zh-CN';

// 存储到 localStorage 的键名
const LANG_STORAGE_KEY = 'ziphere_lang';

// 当前语言
const currentLanguage = ref<LangType>(
  (localStorage.getItem(LANG_STORAGE_KEY) as LangType) || defaultLang
);

/**
 * 格式化文本，支持 {0}, {1} 等占位符
 * @param text 待格式化的文本
 * @param args 参数列表
 * @returns 格式化后的文本
 */
function formatText(text: string, args?: any[]): string {
  if (!args || args.length === 0) return text;
  
  return text.replace(/{(\d+)}/g, (match, index) => {
    const i = Number(index);
    return i < args.length ? String(args[i]) : match;
  });
}

/**
 * 国际化 hook
 * @returns 国际化相关的函数和属性
 */
export function useI18n() {
  // 获取国际化文本
  const t = (key: string, args?: any[]) => {
    // 分割嵌套键，如 'formatCapabilities.compression'
    const keys = key.split('.');
    let result: any = i18nTexts[currentLanguage.value];
    
    // 遍历键路径
    for (const k of keys) {
      if (result && typeof result === 'object' && k in result) {
        result = result[k];
      } else {
        // 如果键不存在，回退到英文
        result = getNestedValue(i18nTexts.en, keys);
        break;
      }
    }
    
    if (typeof result === 'string') {
      return formatText(result, args);
    }
    
    // 如果没有找到，返回键名
    return formatText(key, args);
  };
  
  // 获取嵌套对象的值
  const getNestedValue = (obj: any, keys: string[]) => {
    let result = obj;
    for (const key of keys) {
      if (result && typeof result === 'object' && key in result) {
        result = result[key];
      } else {
        return undefined;
      }
    }
    return result;
  };
  
  // 切换语言
  const changeLang = (lang: LangType) => {
    currentLanguage.value = lang;
    localStorage.setItem(LANG_STORAGE_KEY, lang);
  };
  
  return {
    // 翻译函数
    t,
    // 当前语言
    currentLang: currentLanguage,
    // 语言切换函数
    changeLang,
    // 支持的语言列表
    supportedLangs: computed(() => Object.keys(i18nTexts) as LangType[])
  };
}