import { useLocation, useNavigate } from "solid-app-router";
import { Component, JSX, ParentComponent } from "solid-js";
import { FilmIcon, GearIcon, HomeIcon, TvIcon } from "./icons";

export const NavDrawer: Component = () => {
  const location = useLocation();
  const navigate = useNavigate();

  function isPath(path: string) {
    return location.pathname === path;
  }

  function onNavigate(path: string) {
    if (!isPath(path)) {
      navigate(path);
    }
  }

  const SubTitle: ParentComponent = (p) => {
    return <p class="text-sm font-bold text-gray-400">{p.children}</p>;
  };

  const Section: ParentComponent = (p) => {
    return <div class="p-1">{p.children}</div>;
  };

  interface LinkProps {
    title: string;
    path: string;
    icon: JSX.Element;
  }

  const Link: Component<LinkProps> = (p) => {
    return (
      <div
        class="my-1 p-3 flex items-center select-none text-base transition-colors rounded-lg cursor-pointer [&:not(.active)]:hover:bg-black/10 dark:[&:not(.active)]:hover:bg-white/10"
        classList={{
          "bg-black/20 dark:bg-white/20 text-orange-500 active": isPath(p.path),
        }}
        onClick={[onNavigate, p.path]}
      >
        <div class="mr-4">{p.icon}</div>
        {p.title}
      </div>
    );
  };

  const Divider: Component = () => {
    return <div class={"border-t border-neutral-600 my-3"} />;
  };

  return (
    <div class="flex flex-col p-2 bg-neutral-200 dark:bg-neutral-800 shadow-[0px_0px_4px_#aaaaaa] dark:shadow-[0px_0px_4px_#141414]">
      <div class="p-8 flex justify-center">
        <img src="/images/zenith.png" class="w-[64px] h-[88px]" />
      </div>
      <Divider />
      <Section>
        <SubTitle>General</SubTitle>
        <Link title="Home" path="/" icon={<HomeIcon size={20} />} />
      </Section>
      <Section>
        <SubTitle>Libraries</SubTitle>
        <Link title="Movies" path="/movies" icon={<FilmIcon size={20} />} />
        <Link title="Shows" path="/shows" icon={<TvIcon size={20} />} />
      </Section>
      <Section>
        <SubTitle>System</SubTitle>
        <Link title="Settings" path="/settings" icon={<GearIcon size={20} />} />
      </Section>
    </div>
  );
};
