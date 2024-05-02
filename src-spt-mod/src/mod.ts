import path from 'node:path';
import { DependencyContainer } from 'tsyringe';
import { DatabaseServer } from '@spt-aki/servers/DatabaseServer';
import { SaveServer } from '@spt-aki/servers/SaveServer';
import { LogTextColor } from '@spt-aki/models/spt/logging/LogTextColor';
import { Watermark } from '@spt-aki/utils/Watermark';
import { PreAkiModLoader } from '@spt-aki//loaders/PreAkiModLoader';
import type { IPreAkiLoadMod } from '@spt-aki/models/external/IPreAkiLoadMod';
import type { ILogger } from '@spt-aki/models/spt/utils/ILogger';
import type { StaticRouterModService } from '@spt-aki/services/mod/staticRouter/StaticRouterModService';

class TarkovStash implements IPreAkiLoadMod {
  public preAkiLoad(container: DependencyContainer): void {
    const logger = container.resolve<ILogger>('WinstonLogger');
    const databaseServer = container.resolve<DatabaseServer>('DatabaseServer');
    const saveServer = container.resolve<SaveServer>('SaveServer');
    const watermark = container.resolve<Watermark>('Watermark');
    const preAkiModLoader = container.resolve<PreAkiModLoader>('PreAkiModLoader');

    const staticRouterModService =
      container.resolve<StaticRouterModService>('StaticRouterModService');

    // Hook up a new static route
    staticRouterModService.registerStaticRouter(
      'TarkovStashModRouter',
      [
        {
          url: '/tarkov-stash/server',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading server info`, LogTextColor.GREEN);
            const version = watermark.getVersionTag();
            const serverPath = path.resolve();
            const tarkovStashMod = preAkiModLoader.getImportedModDetails()['tarkov-stash'];
            const modVersion = tarkovStashMod.version;
            return JSON.stringify({ version, path: serverPath, modVersion });
          },
        },
        {
          url: '/tarkov-stash/profiles',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading profiles`, LogTextColor.GREEN);
            return JSON.stringify(saveServer.getProfiles());
          },
        },
        {
          url: '/tarkov-stash/profile',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading profile [${sessionId}]`, LogTextColor.GREEN);
            return JSON.stringify(saveServer.getProfile(sessionId));
          },
        },
        {
          url: '/tarkov-stash/reload-profile',
          action: (url, info, sessionId, output) => {
            logger.log(
              `[tarkov-stash] Refreshing profile [${sessionId}] from disk`,
              LogTextColor.GREEN,
            );
            saveServer.loadProfile(sessionId);
            return 'ok';
          },
        },
        {
          url: '/tarkov-stash/items',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading items`, LogTextColor.GREEN);
            return JSON.stringify(databaseServer.getTables().templates.items);
          },
        },
        {
          url: '/tarkov-stash/globals-presets',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading global presets`, LogTextColor.GREEN);
            return JSON.stringify(databaseServer.getTables().globals.ItemPresets);
          },
        },
      ],
      'tarkov-stash-top-level-route',
    );
  }
}

module.exports = { mod: new TarkovStash() };
