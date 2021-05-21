import { ColorSchemeName } from "react-native";
import { DarkTheme, DefaultTheme } from "@react-navigation/native";

export function navigationTheme(scheme: ColorSchemeName) {
    return scheme === "dark" ? DarkTheme : DefaultTheme;
}
