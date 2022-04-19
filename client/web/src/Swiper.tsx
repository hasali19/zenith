import { Component, onCleanup, onMount } from "solid-js";
import SwiperClass from "swiper";

import "swiper/css";

export const Swiper: Component = (p) => {
  let el!: HTMLDivElement;
  let swiper!: SwiperClass;

  onMount(() => {
    swiper = new SwiperClass(el, {
      threshold: 10,
      slidesPerView: 2,
      slidesPerGroup: 2,
      breakpoints: {
        480: { slidesPerView: 3, slidesPerGroup: 3 },
        640: { slidesPerView: 5, slidesPerGroup: 5 },
        1024: { slidesPerView: 6, slidesPerGroup: 6 },
        1400: { slidesPerView: 8, slidesPerGroup: 8 },
      },
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

export const SwiperSlide: Component = (p) => (
  <div class="swiper-slide">{p.children}</div>
);
