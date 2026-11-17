using System.Runtime.InteropServices;

namespace PocketKnives;

internal static class RunnerLink
{
    internal const string OpenMessage = "PocketKnives.OpenSettings.v1";
    internal const string ReopenMessage = "PocketKnives.ReopenSettings.v1";
    internal const string ExitMessage = "PocketKnives.Exit.v1";

    [DllImport("user32.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
    internal static extern nint FindWindowW(string? className, string? title);
    [DllImport("user32.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
    internal static extern uint RegisterWindowMessageW(string name);
    [DllImport("user32.dll")] internal static extern int PostMessageW(nint window, uint message, nuint wParam, nint lParam);

    internal static bool Post(nint window, string message) =>
        window != 0 && PostMessageW(window, RegisterWindowMessageW(message), 0, 0) != 0;
}
