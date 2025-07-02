import { useState } from "react";

const TtsSettings = ({ languages, voices, tts, setTts }) => {
  const [the_language, set_the_language] = useState("en-US");
  return (
    <div className="flex flex-col items-center gap-2">
      <div className="flex flex-row items-center gap-2">
        <label>TTS</label>
        <input
          type="checkbox"
          checked={tts.enabled}
          id="ttsenable"
          onChange={(e) => setTts({ ...tts, enabled: e.target.checked })}
        />
        <label htmlFor="language">language</label>
        <select
          id="language"
          value={the_language}
          onChange={(e) => {set_the_language(e.target.value); setTts({ ...tts, voice: voices[e.target.value][0]})}}
        >
          {languages.map((lang) => (
            <option key={lang} value={lang}>
              {lang}
            </option>
          ))}
        </select>
        <label htmlFor="voice">voice</label>
        <select
          id="voice"
          value={tts.voice}
          onChange={(e) => setTts({ ...tts, voice: e.target.value })}
        >
          {voices[the_language].map((voice) => (
            <option key={voice} value={voice}>
              {voice}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
};

export default TtsSettings;
