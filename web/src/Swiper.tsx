import { onCleanup, onMount, ParentComponent } from "solid-js";
import SwiperClass, { SwiperOptions } from "swiper";

import "swiper/css";

export interface SwiperProps {
  breakpoints: {
    [width: number]: SwiperOptions;
  };
}

export const Swiper: ParentComponent<SwiperProps> = (p) => {
  let el!: HTMLDivElement;
  let swiper!: SwiperClass;

  onMount(() => {
    swiper = new SwiperClass(el, {
      threshold: 10,
      slidesPerView: 2,
      slidesPerGroup: 2,
      breakpoints: p.breakpoints,
    });
  });

  onCleanup(() => {
    swiper.destroy();
  });

  return (
    <div ref={el} class="swiper">
      <div class="swiper-wrapper">{p.children}</div>
    </div>
  );
};

export const SwiperSlide: ParentComponent = (p) => (
  <div class="swiper-slide">{p.children}</div>
);
