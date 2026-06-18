<template>
  <div class="sidebar-panel">
    <div
      v-for="smartAlbum in systemSmartAlbums"
      :key="smartAlbum.id"
      :class="[
        'sidebar-item',
        libConfig.smartAlbum.type === 'system' && libConfig.smartAlbum.id === smartAlbum.id ? 'sidebar-item-selected' : 'sidebar-item-hover',
      ]"
      @click="clickSmartAlbum(smartAlbum)"
    >
      <component :is="smartAlbum.icon" class="mx-1 w-5 h-5 shrink-0" />
      <div class="sidebar-item-label">
        <span>{{ smartAlbum.label }}</span>
      </div>
    </div>

    <div class="sidebar-panel-header">
      <span class="sidebar-panel-header-title flex-1">{{ $t('album.smart_album_list') }}</span>
      <TButton
        :icon="IconAdd"
        :buttonSize="'small'"
        :tooltip="$t('album.add_smart_album')"
        @click="clickAddSmartAlbum"
      />
    </div>

    <ul
      v-if="customSmartAlbums.length > 0"
      class="flex-1 overflow-x-hidden overflow-y-auto rounded-box select-none"
    ></ul>
    <div v-else class="mt-2 px-2 flex flex-col items-center justify-center text-base-content/30">
      <span class="text-sm text-center">{{ $t('album.no_smart_albums.description') }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { libConfig } from '@/common/config';
import { IconAdd, IconCalendarDay, IconHistory } from '@/common/icons';
import TButton from '@/components/TButton.vue';

const { locale, messages } = useI18n();
const localeMsg = computed(() => messages.value[locale.value] as any);

type SystemSmartAlbum = {
  id: 'recently-added' | 'on-this-day';
  label: string;
  icon: any;
};

const systemSmartAlbums = computed<SystemSmartAlbum[]>(() => [
  {
    id: 'recently-added',
    label: localeMsg.value.album.smart_items.recently_added,
    icon: IconHistory,
  },
  {
    id: 'on-this-day',
    label: localeMsg.value.album.smart_items.on_this_day,
    icon: IconCalendarDay,
  },
]);
const customSmartAlbums = computed(() => []);

function clickSmartAlbum(smartAlbum: SystemSmartAlbum) {
  libConfig.smartAlbum.type = 'system';
  libConfig.smartAlbum.id = smartAlbum.id;
}

function clickAddSmartAlbum() {
  // Placeholder until custom smart album queries are wired up.
}
</script>
