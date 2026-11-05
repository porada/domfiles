# Preserve Pending Fixups

Use the base’s history route with these requirements for fixups already present in the selected history. Creating a fixup does not authorize its consolidation during a later update.

Before replay or consolidation, identify each pending fixup’s target and record its authorized disposition in the execution baseline. A full object ID identifies the original target, not a replacement created by replay. If an earlier rewrite changed that ID, resolve the replacement from an old-to-new mapping or verified patch evidence. An uncertain match stops the affected update. Do not guess from a repeated subject.

Consolidate only fixups covered by the history-update authorization. Preserve other pending fixups as separate commits, overriding autosquash defaults or generated todo actions as needed. When replay changes a preserved fixup’s target ID, include retargeting its subject in the authorized update and record the expected relationship before execution. Resolve any uncovered retargeting effect through the base’s confirmation step before replay.

Verify preserved fixups against their resulting targets and contents. Distinguish them from temporary fixups created for the update, which must be consumed as planned. Check that only authorized fixups were consolidated and that every preserved fixup still identifies its intended target.
