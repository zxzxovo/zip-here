import langKeys from "./langKeys";

export type LangType = "zh-CN" | "en"

// 支持嵌套对象的国际化键类型
export type I18nKeyType = { 
    [key in langKeys]: string | Record<string, string>  
};

export type I18nType = {
    [key in LangType]: I18nKeyType;
} & I18nKeyType;