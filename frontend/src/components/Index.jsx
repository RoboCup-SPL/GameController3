import { useState } from "react";
import Launcher from "./Launcher";
import Main from "./Main";

const Index = () => {
  const [launched, setLaunched] = useState(false);

  if (launched) {
    return <Main />;
  } else {
    return <Launcher setLaunched={setLaunched} />;
  }
};

export default Index;
