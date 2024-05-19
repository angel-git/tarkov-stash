using Aki.Reflection.Utils;
using BepInEx;
using Comfort.Common;
using EFT.Communications;
using EFT.UI.DragAndDrop;
using EFT.UI;
using EFT;
using HarmonyLib;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;

using TarkovStash.Patches;

namespace TarkovStash
{
    [BepInPlugin("com.agavalda.TarkovStash", "TarkovStash", "1.0.0")]
    public class Plugin : BaseUnityPlugin
    {
        public static Plugin Instance;

        private TarkovApplication tarkovApplication => ClientAppUtils.GetMainApp();

        private FieldInfo _mainMenuControllerField => typeof(TarkovApplication).GetFields(PatchConstants.PrivateFlags).FirstOrDefault(x => x.FieldType == typeof(MainMenuController));
        private void Awake()
        {
            // Plugin startup logic
            Logger.LogInfo($"Plugin TarkovStash is loaded!");
            Instance = this;

            new NotificationManagerClassPatch().Enable();
        }

        private MainMenuController GetMainMenuController()
        {
            return _mainMenuControllerField.GetValue(tarkovApplication) as MainMenuController;
        }

        public void handleMessage(string message)
        {
            if (message.Contains("\"type\":\"test-stash\""))
            {
                Logger.LogInfo($"refresh message received!");

                GetMainMenuController().OnProfileChangeApplied(ENotificationRequirements.ReloadProfile);
            }
        }
    }
}
