using Aki.Reflection.Patching;
using HarmonyLib;
using System.Reflection;

namespace TarkovStash.Patches
{

    internal class NotificationManagerClassPatch : ModulePatch
    {


        protected override MethodBase GetTargetMethod()
        {
            return AccessTools.Method(typeof(NotificationManagerClass), nameof(NotificationManagerClass.method_7));
        }

        [PatchPostfix]
        public static void PatchPostfix(string message)
        {
            Plugin.Instance.handleMessage(message);
        }

    }

}