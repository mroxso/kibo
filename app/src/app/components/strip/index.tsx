import { AnchorAPI } from "./components/anchorAPI";
import { AnchorGeyser } from "./components/anchorGeyser";
import { AnchorGit } from "./components/anchorGit";
import { AnchorLogo } from "./components/anchorLogo";
import { AnchorNostr } from "./components/anchorNostr";
import { ButtonChart } from "./components/buttonChart";
import { ButtonFavorites } from "./components/buttonFavorites";
import { ButtonFolders } from "./components/buttonFolders";
import { ButtonHistory } from "./components/buttonHistory";
import { ButtonSearch } from "./components/buttonSearch";
import { ButtonSettings } from "./components/buttonSettings";

export function StripDesktop({
  selected,
  setSelected,
}: {
  selected: Accessor<FrameName>;
  setSelected: Setter<FrameName>;
}) {
  return (
    <>
      <AnchorLogo />

      <ButtonFolders selected={selected} setSelected={setSelected} />
      <ButtonFavorites selected={selected} setSelected={setSelected} />
      <ButtonSearch selected={selected} setSelected={setSelected} />
      <ButtonHistory selected={selected} setSelected={setSelected} />

      <ButtonSettings selected={selected} setSelected={setSelected} />

      <div class="size-full" />

      <AnchorAPI />
      <AnchorGit />
      <AnchorNostr />
      <AnchorGeyser />
      {/* <AnchorHome /> */}
    </>
  );
}

export function StripMobile({
  selected,
  setSelected,
}: {
  selected: Accessor<FrameName>;
  setSelected: Setter<FrameName>;
}) {
  return (
    <>
      <ButtonChart selected={selected} setSelected={setSelected} />
      <ButtonFolders selected={selected} setSelected={setSelected} />
      <ButtonFavorites selected={selected} setSelected={setSelected} />
      <ButtonSearch selected={selected} setSelected={setSelected} />
      <ButtonHistory selected={selected} setSelected={setSelected} />
      <ButtonSettings selected={selected} setSelected={setSelected} />
    </>
  );
}
