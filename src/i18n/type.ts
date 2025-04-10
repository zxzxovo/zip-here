import langKeys from "./langKeys";

export type LangType = "zh-CN" | "en"

export type I18nKeyType = { 
    [key in langKeys]: string  
};

export type I18nType = {
    [key in LangType]: I18nKeyType;
} & I18nKeyType;