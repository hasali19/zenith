import { iso6392 } from "iso-639-2";

export const languages: Record<string, string> = iso6392.reduce(
  (languages, language) => ({
    ...languages,
    [language.iso6392B]: language.name.split(";")[0].trim(),
  }),
  {}
);
