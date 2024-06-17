import path from 'node:path';
import { DependencyContainer } from 'tsyringe';
import { DatabaseServer } from '@spt/servers/DatabaseServer';
import { SaveServer } from '@spt/servers/SaveServer';
import { LogTextColor } from '@spt/models/spt/logging/LogTextColor';
import { Watermark } from '@spt/utils/Watermark';
import { PreSptModLoader } from '@spt/loaders/PreSptModLoader';
import { SptWebSocketConnectionHandler } from '@spt/servers/ws/SptWebSocketConnectionHandler';
import type { IPreSptLoadMod } from '@spt/models/external/IPreSptLoadMod';
import type { ILogger } from '@spt/models/spt/utils/ILogger';
import type { StaticRouterModService } from '@spt/services/mod/staticRouter/StaticRouterModService';

class TarkovStash implements IPreSptLoadMod {
  public preSptLoad(container: DependencyContainer): void {
    const logger = container.resolve<ILogger>('WinstonLogger');
    const databaseServer = container.resolve<DatabaseServer>('DatabaseServer');
    const saveServer = container.resolve<SaveServer>('SaveServer');
    const watermark = container.resolve<Watermark>('Watermark');
    const preAkiModLoader = container.resolve<PreSptModLoader>('PreSptModLoader');
    const webSocketServer = container.resolve<SptWebSocketConnectionHandler>(
      'SptWebSocketConnectionHandler',
    );

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
            const modsInstalled = Object.values(preAkiModLoader.getImportedModDetails());
            const tarkovStashMod = modsInstalled.find((m) => m.name === 'tarkov-stash');
            const modVersion = tarkovStashMod?.version;
            return Promise.resolve(JSON.stringify({ version, path: serverPath, modVersion }));
          },
        },
        {
          url: '/tarkov-stash/profiles',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading profiles`, LogTextColor.GREEN);
            return Promise.resolve(JSON.stringify(saveServer.getProfiles()));
          },
        },
        {
          url: '/tarkov-stash/profile',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading profile [${sessionId}]`, LogTextColor.GREEN);
            return Promise.resolve(JSON.stringify(saveServer.getProfile(sessionId)));
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
            webSocketServer.sendMessage(sessionId, { type: 'tarkov-stash-reload', eventId: '' });
            return Promise.resolve('ok');
          },
        },
        {
          url: '/tarkov-stash/items',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading items`, LogTextColor.GREEN);
            return Promise.resolve(JSON.stringify(databaseServer.getTables().templates.items));
          },
        },
        {
          url: '/tarkov-stash/globals-presets',
          action: (url, info, sessionId, output) => {
            logger.log(`[tarkov-stash] Loading global presets`, LogTextColor.GREEN);
            return Promise.resolve(JSON.stringify(databaseServer.getTables().globals.ItemPresets));
          },
        },
      ],
      'tarkov-stash-top-level-route',
    );
  }
}

module.exports = { mod: new TarkovStash() };
